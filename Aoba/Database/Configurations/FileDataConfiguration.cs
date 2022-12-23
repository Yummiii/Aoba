using Aoba.Database.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Aoba.Database.Configurations;

public class FileDataConfiguration: IEntityTypeConfiguration<FileData>
{
    public void Configure(EntityTypeBuilder<FileData> builder)
    {
        builder.ToTable("FilesData");
        builder.HasKey(x => x.Id);
        builder.Property(x => x.Content).IsRequired().HasColumnType("LONGTEXT");
        builder.Property(x => x.MimeType).IsRequired().HasMaxLength(255);
    }
}