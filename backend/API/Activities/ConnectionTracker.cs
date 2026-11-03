namespace KeepGrouped.API.Activities;

public class ConnectionTracker
{
	private readonly Dictionary<string, int> connections = [];

	public void Add(string userId)
	{
		lock (connections)
		{
			connections[userId] = connections.GetValueOrDefault(userId) + 1;
		}
	}

	public bool Remove(string userId)
	{
		lock (connections)
		{
			var remaining = connections.GetValueOrDefault(userId) - 1;
			if (remaining > 0)
			{
				connections[userId] = remaining;
				return false;
			}
			connections.Remove(userId);
			return true;
		}
	}
}
