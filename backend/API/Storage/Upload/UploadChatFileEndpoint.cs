using System.ComponentModel.DataAnnotations;
using System.Net.Mime;
using KeepGrouped.API.AiBackend.Ingest;
using KeepGrouped.API.Attributes.Roles;
using KeepGrouped.API.Middlewares;
using KeepGrouped.API.Problems;
using KeepGrouped.API.Rag;
using KeepGrouped.API.Roles;
using KeepGrouped.API.Users;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace KeepGrouped.API.Storage;

public record UploadChatFileRequest
{
    [Required]
    [Length(1, 255)]
    [RegularExpression(NulFreeStringConverter.Pattern, ErrorMessage = NulFreeStringConverter.ErrorMessage)]
    public string Name { get; init; } = null!;

    [Required]
    public IFormFile File { get; init; } = null!;
}

public record UploadChatFileResponse(
    string Key,
    string Name,
    long Length,
    string ETag,
    string ContentType,
    DateTime LastUpdated,
    UserSummary Creator
)
{
    public static UploadChatFileResponse FromEntity(StorageFile file) => new(
        file.Key, file.Name, file.Length, file.ETag, file.ContentType,
        file.LastUpdated, UserSummary.FromEntity(file.Creator));
}

public static class UploadChatFileEndpoint
{
    public static void MapChatUploadFile(this IEndpointRouteBuilder group)
    {
        group.MapPost("/chat", [Authorize] async (UploadChatFileCommand uploadCmd, TokenContext token,
            [FromForm] UploadChatFileRequest req, CancellationToken ct) =>
        {
            var uploadResult = await uploadCmd.ExecuteAsync(req.File.OpenReadStream(), req.Name, req.File.ContentType, req.File.Length, token.User, ct);

            if (uploadResult.IsProblem)
            {
                return uploadResult.Problem;
            }
            return Results.CreatedAtRoute("files.get", new { key = uploadResult.Value.Key }, UploadFileResponse.FromEntity(uploadResult.Value));
        })
        .DisableAntiforgery()
        .WithName("chatfiles.upload")
        .WithSummary("Upload a chat file")
        .WithDescription("Uploads a chat file as `multipart/form-data` (fields `name` and `file`) and stores its metadata in the database.")
        .Accepts<UploadFileRequest>("multipart/form-data")
        .Produces<UploadFileResponse>(StatusCodes.Status201Created)
        .ProducesValidationProblem()
        .ProducesProblem(StatusCodes.Status401Unauthorized)
        .ProducesProblem(StatusCodes.Status422UnprocessableEntity)
        .ProducesProblem(StatusCodes.Status502BadGateway);
    }
}
