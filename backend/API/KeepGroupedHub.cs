using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.SignalR;
using Microsoft.EntityFrameworkCore;
using KeepGrouped.API.Activities;
using KeepGrouped.API.Channels;
using KeepGrouped.API.Users;

namespace KeepGrouped.API;

static class MessagesDb
{
	public static async Task<User?> GetUser(KeepGroupedDb db, string? id) => await db.Users
			.Include(user => user.Role)
			.Include(user => user.Avatar)
			.SingleOrDefaultAsync(user => user.Id == id);
}

[Authorize]
public class KeepGroupedHub(KeepGroupedDb db, ConnectionTracker connections) : Hub
{
	public override async Task OnConnectedAsync()
	{
		if (Context.UserIdentifier is { } userId)
		{
			connections.Add(userId);
		}
		await base.OnConnectedAsync();
	}

	public override async Task OnDisconnectedAsync(Exception? exception)
	{
		if (Context.UserIdentifier is { } userId && connections.Remove(userId))
		{
			await Clients.All.SendAsync("OtherActivityUpdated", userId, ActivityEnum.Offline);
		}
		await base.OnDisconnectedAsync(exception);
	}

	public async Task ActivityReported(ActivityEnum activity)
	{
		var userId = Context.UserIdentifier;

		await Clients.Others.SendAsync("OtherActivityUpdated", userId, activity);
	}

	public async Task ReportActivityTo(string userId, ActivityEnum activity)
	{
		await Clients.User(userId).SendAsync("OtherActivityUpdated", Context.UserIdentifier, activity);
	}

	public async Task AskOthersActivity()
	{
		await Clients.Others.SendAsync("ReportActivityTo", Context.UserIdentifier);
	}

	//  Messages request

	public async Task<MessageResponse> CreateMessage(CreateMessageCommand command, string id, CreateMessageRequest req)
	{
		var user = await MessagesDb.GetUser(db, Context.UserIdentifier)
			?? throw new HubException("USER_NOT_AUTHENTICATED");

		var result = await command.ExecuteAsync(id, user, req);
		if (result.IsProblem)
		{
			throw new HubException(result.Problem.ProblemDetails.Extensions["errorCode"]?.ToString());
		}
		return result.Value;
	}

	public async Task UpdateMessage(UpdateMessageCommand command, string id, string msgId, UpdateMessageRequest req)
	{
		var user = await MessagesDb.GetUser(db, Context.UserIdentifier)
			?? throw new HubException("USER_NOT_AUTHENTICATED");

		var result = await command.ExecuteAsync(id, msgId, user, req);
		if (result.IsProblem)
		{
			throw new HubException(result.Problem.ProblemDetails.Extensions["errorCode"]?.ToString());
		}
	}

	public async Task DeleteMessage(DeleteMessageCommand command, string id, string msgId)
	{
		var user = await MessagesDb.GetUser(db, Context.UserIdentifier)
			?? throw new HubException("USER_NOT_AUTHENTICATED");

		var result = await command.ExecuteAsync(id, msgId, user);
		if (result.IsProblem)
		{
			throw new HubException(result.Problem.ProblemDetails.Extensions["errorCode"]?.ToString());
		}
	}
}
