using Microsoft.AspNetCore.Http.HttpResults;

namespace KeepGrouped.API.Problems;

static public class RoleProblems
{
    static public ProblemHttpResult NotFound(string id) => ProblemFactory.Create(
        StatusCodes.Status404NotFound, "ROLE_NOT_FOUND",
        "Role not found",
        $"No role has the id '{id}'.");

    static public ProblemHttpResult NameAlreadyUsed(string name) => ProblemFactory.Create(
    StatusCodes.Status409Conflict, "NAME_ALREADY_USED",
    "Role Name already used",
    $"'{name}' is already used by an other role.");

    static public ProblemHttpResult InvalidPermission(int perm) => ProblemFactory.Create(
    StatusCodes.Status400BadRequest, "INVALID_PERMISSION",
    "Invalid permission",
    $"'{perm}' contains unknown permission bits.");

    static public ProblemHttpResult NotFound() => ProblemFactory.Create(
   StatusCodes.Status404NotFound, "ROLE_NOT_FOUND",
   "Role not found",
   $"No role has the id.");

    static public ProblemHttpResult NotAllowed() => ProblemFactory.Create(
    StatusCodes.Status404NotFound, "NOT_ALLOWED",
    "Not allowed",
    $"Not allowed to remove the role member");

    static public ProblemHttpResult RankingTooLow() => ProblemFactory.Create(
  StatusCodes.Status403Forbidden, "RANKING_TOO_LOW",
  "Ranking too low", "Your actual role ranking is too low to doing this action");

    static public ProblemHttpResult InvalidDirection(string direction) => ProblemFactory.Create(
        StatusCodes.Status400BadRequest, "INVALID_DIRECTION",
        "Invalid direction",
        $"'{direction}' is not a valid direction, expected 'up' or 'down'.");

    static public ProblemHttpResult RankOutOfBounds() => ProblemFactory.Create(
        StatusCodes.Status409Conflict, "RANK_OUT_OF_BOUNDS",
        "Cannot move role",
        "The role is already at the edge of the hierarchy.");

    static public ProblemHttpResult ProtectedRole() => ProblemFactory.Create(
        StatusCodes.Status403Forbidden, "PROTECTED_ROLE",
        "Protected role",
        "This role cannot be renamed.");

}
