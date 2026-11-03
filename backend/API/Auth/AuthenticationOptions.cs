using System.ComponentModel.DataAnnotations;

namespace KeepGrouped.API.Users.Auth;

public class AuthenticationOptions
{
    public const string SectionName = "Authentication";

    [Required]
    [Length(1, 255)]
    [RegularExpression(@"^[a-zA-Z0-9_]+$", ErrorMessage = "DefaultAdminLogin can only contain letters, numbers, and underscores (_).")]
    public required string DefaultAdminLogin { get; init; }

    [Required]
    [EmailAddress]
    [Length(3, 255)]
    public required string DefaultAdminEmail { get; init; }

    [Required]
    [Length(8, 255)]
    public required string DefaultAdminPwd { get; init; }

    // 256 bits random key: openssl rand -hex 32
    [Required]
    [RegularExpression("^[0-9a-fA-F]{64}$", ErrorMessage = "JWTKey must be 64 hex chars (openssl rand -hex 32).")]
    public required string JWTKey { get; init; }
}
