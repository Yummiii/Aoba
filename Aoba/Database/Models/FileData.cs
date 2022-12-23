namespace Aoba.Database.Models;

public class FileData
{
    public ulong Id { get; set; }
    public string MimeType { get; set; }
    public string Content { get; set; }

    public FileMetadata File { get; set; }
}