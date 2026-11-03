using System.ComponentModel.DataAnnotations;

namespace KeepGrouped.API.Storage;

public class StorageOptions
{
    public const string SectionName = "Storage";

    [Required]
    [Url]
    public required string ServiceUrl { get; init; }

    [Required]
    public required string Region { get; init; }

    [Required]
    [RegularExpression(@"^[a-z0-9][a-z0-9.-]{1,61}[a-z0-9]$", ErrorMessage = "BucketName must be a valid S3 bucket name.")]
    public required string BucketName { get; init; }

    // "GK" followed by 24 hex chars: echo "GK$(openssl rand -hex 12)"
    [Required]
    [RegularExpression("^GK[0-9a-fA-F]{24}$", ErrorMessage = "AccessKey must be \"GK\" followed by 24 hex chars.")]
    public required string AccessKey { get; init; }

    // 64 hex chars: openssl rand -hex 32
    [Required]
    [RegularExpression("^[0-9a-fA-F]{64}$", ErrorMessage = "SecretKey must be 64 hex chars (openssl rand -hex 32).")]
    public required string SecretKey { get; init; }
}
