using Aoba.Database.Configurations;
using Aoba.Database.Models;
using Microsoft.EntityFrameworkCore;

namespace Aoba.Database;

public class AppDbContext: DbContext
{
    public DbSet<User> Users { get; set; }
    public DbSet<FileData> FilesData { get; set; }
    public DbSet<FileMetadata> Files { get; set; }
    
    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        string conn = "Server=localhost;Database=Aoba;User Id=root;Password=root;Port=3306";
        optionsBuilder.UseMySql(conn, ServerVersion.AutoDetect(conn));
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.ApplyConfiguration(new UserConfigurations());
        modelBuilder.ApplyConfiguration(new FileConfiguration());
        modelBuilder.ApplyConfiguration(new FileDataConfiguration());
    }
}