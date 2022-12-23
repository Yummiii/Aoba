using NCuid;

namespace Aoba.Database.Models;

public class User
{
    public string Id { get; set; } = Cuid.Generate();
    public string Username { get; set; }
    public string Password { get; set; }
    public string LastToken { get; set; }
    
    public IEnumerable<FileMetadata> Files { get; set; }
}