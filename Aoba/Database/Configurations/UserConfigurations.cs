using Aoba.Database.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Aoba.Database.Configurations;

public class UserConfigurations: IEntityTypeConfiguration<User>
{
    public void Configure(EntityTypeBuilder<User> builder)
    {
        builder.ToTable("Users");
        builder.HasKey(x => x.Id);
        builder.Property(x => x.Id).HasMaxLength(100);
        builder.Property(x => x.Username).HasMaxLength(255).IsRequired();
        builder.Property(x => x.Password).HasMaxLength(100).IsRequired();
        builder.HasIndex(x => x.Username).IsUnique();
    }
}