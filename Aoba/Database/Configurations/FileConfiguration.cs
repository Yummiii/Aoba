using Aoba.Database.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Aoba.Database.Configurations;

public class FileConfiguration: IEntityTypeConfiguration<FileMetadata>
{
    public void Configure(EntityTypeBuilder<FileMetadata> builder)
    {
        builder.ToTable("Files");
        builder.HasKey(x => x.Id);
        builder.Property(x => x.Id).HasMaxLength(100);
        builder.Property(x => x.Name).HasMaxLength(255).IsRequired();
        builder.Property(x => x.CreatedAt).IsRequired();
        builder.Property(x => x.IsDirectory).IsRequired();
        builder.Property(x => x.IsPublic).IsRequired();
        builder.Property(x => x.IsListed).IsRequired();
        
        builder.Property(x => x.ParentId).HasMaxLength(100);
        builder.Property(x => x.OwnerId).HasMaxLength(100).IsRequired();
        builder.Property(x => x.DataId).HasMaxLength(100);

        builder.HasOne(x => x.Parent).WithMany(x => x.Children).HasForeignKey(x => x.ParentId);
        builder.HasOne(x => x.Owner).WithMany(x => x.Files).HasForeignKey(x => x.OwnerId);
        builder.HasOne(x => x.Data).WithOne(x => x.File).HasForeignKey<FileMetadata>(x => x.DataId);
    }
}