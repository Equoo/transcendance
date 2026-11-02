using KeepGrouped.API;
using KeepGrouped.API.Middlewares;
using KeepGrouped.API.Problems;
using KeepGrouped.API.Roles;
using Microsoft.EntityFrameworkCore;

public sealed class RankingControlCommand(KeepGroupedDb db, TokenContext tk) : IHandler
{
    public async Task<Result> ExecAsync(string id, string direction)
    {
        if (direction != "up" && direction != "down")
        {
            return RoleProblems.InvalidDirection(direction);
        }

        List<Role> roles = await db.Roles
            .Where(r => r.Name != Role.SuperRole)
            .OrderBy(r => r.Ranking)
            .ThenBy(r => r.Name)
            .ToListAsync();

        int index = roles.FindIndex(r => r.Id == id);

        if (index == -1)
        {
            return RoleProblems.NotFound(id);
        }

        int actor = roles.FindIndex(r => r.Id == tk.User.Role.Id);

        if (index <= actor)
        {
            return RoleProblems.RankingTooLow();
        }

        int target = direction == "up" ? index - 1 : index + 1;

        if (target < 0 || target >= roles.Count)
        {
            return RoleProblems.RankOutOfBounds();
        }

        if (target <= actor)
        {
            return RoleProblems.RankingTooLow();
        }

        (roles[index], roles[target]) = (roles[target], roles[index]);

        for (int i = 0; i < roles.Count; i++)
        {
            roles[i].Ranking = i + 1;
        }

        await db.SaveChangesAsync();

        return Result.OK;
    }
}
