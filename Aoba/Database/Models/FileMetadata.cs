using NCuid;

namespace Aoba.Database.Models;

public class FileMetadata
{
    public string Id { get; set; } = Cuid.Generate();
    public string Name { get; set; }
    public long CreatedAt { get; set; } = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
    public bool IsDirectory { get; set; }
    public bool IsPublic { get; set; }
    public bool IsListed { get; set; }

    public string ParentId { get; set; }
    public FileMetadata Parent { get; set; }
    public string OwnerId { get; set; }
    public User Owner { get; set; }
    public ulong DataId {get; set; }
    public FileData Data { get; set; }
    
    public IEnumerable<FileMetadata> Children { get; set; }
}