
using KeepGrouped.API.Middlewares;
using KeepGrouped.API.Problems;
using Microsoft.EntityFrameworkCore;

namespace KeepGrouped.API.Roles;

public sealed class CreateRoleCommand(KeepGroupedDb db) : IHandler
{
    public async Task<Result> ExecAsync(string name)
    {

        Role? role_db = await db.Roles.FirstOrDefaultAsync(r => r.Name.ToLower() == name.ToLower());

        if (role_db is not null)
        {
            return RoleProblems.NameAlreadyUsed(name);
        }

        int last_rank = await db.Roles.MaxAsync(r => (int?)r.Ranking) ?? 0;

        Role role = new(name, last_rank + 1);

        db.Roles.Add(role);

        await db.SaveChangesAsync();

        return Result.OK;
    }
}