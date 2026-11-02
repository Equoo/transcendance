using KeepGrouped.API.Attributes.Roles;
using KeepGrouped.API.Problems;
using Microsoft.AspNetCore.Http.Features;

namespace KeepGrouped.API.Middlewares;

public class RolesMiddleware
{
	private readonly RequestDelegate _next;

	public RolesMiddleware(RequestDelegate next)
	{
		_next = next;
	}

	public async Task InvokeAsync(HttpContext context, TokenContext tk)
	{
		var endpoint = context.Features.Get<IEndpointFeature>()?.Endpoint;
		RolesAttribute? attribute = endpoint?.Metadata.GetMetadata<RolesAttribute>();
		if (attribute is not null)
		{
			if (!tk.User.Role.Permission.HasFlag(attribute.Permission))
			{
				await UserProblems.MissingPermission().ExecuteAsync(context);
				return;
			}
		}
		await _next(context);
	}
}
