using KeepGrouped.API.Channels;

namespace KeepGrouped.API.Users.Me.Conversations;

public record ConversationResponse(
	string Id,
	ChannelType Type,
	string Name,
	string Topic,
	DateTime CreateAt,
	IReadOnlyList<GetUserResponse> Recipients,
	MessageReference? LastMessage
)
{
	public static ConversationResponse FromEntity(Channel c, string meId) =>
		new(
				c.Id,
				c.Type,
				c.Name,
				c.Topic,
				c.CreateAt,
				[.. c.Members.Where(m => m.UserId != meId).Select(m => GetUserResponse.FromEntity(m.User))],
				c.LastMessage is null ? null : MessageReference.FromEntity(c.LastMessage)
		);
}
