using System.ComponentModel.DataAnnotations;
using Microsoft.Extensions.Options;

namespace KeepGrouped.API.RateLimiting;

public class RateLimitOptions
{
	public const string SectionName = "RateLimit";

	[ValidateObjectMembers]
	public TokenBucketPolicyOptions AiChat { get; set; } = new();
	[ValidateObjectMembers]
	public SlidingWindowPolicyOptions Login { get; set; } = new();
}

public class TokenBucketPolicyOptions
{
	[Range(1, int.MaxValue)]
	public int TokenLimit { get; set; }
	[Range(0, int.MaxValue)]
	public int QueueLimit { get; set; }
	[Range(double.Epsilon, double.MaxValue)]
	public double ReplenishmentPeriod { get; set; }
	[Range(1, int.MaxValue)]
	public int TokensPerPeriod { get; set; }
	public bool AutoReplenishment { get; set; }
}

public class SlidingWindowPolicyOptions
{
	[Range(1, int.MaxValue)]
	public int PermitLimit { get; set; }
	[Range(double.Epsilon, double.MaxValue)]
	public double WindowSeconds { get; set; }
	[Range(1, int.MaxValue)]
	public int SegmentsPerWindow { get; set; }
}
