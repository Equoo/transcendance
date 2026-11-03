using System.ComponentModel.DataAnnotations;

public class AiBackendOptions
{
	public const string SectionName = "Ai";

	[Required]
	[Url]
	public string ServiceUrl { get; init; } = null!;
}
