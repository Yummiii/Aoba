﻿// <auto-generated />
using Aoba.Database;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Infrastructure;
using Microsoft.EntityFrameworkCore.Storage.ValueConversion;

#nullable disable

namespace Aoba.Migrations
{
    [DbContext(typeof(AppDbContext))]
    partial class AppDbContextModelSnapshot : ModelSnapshot
    {
        protected override void BuildModel(ModelBuilder modelBuilder)
        {
#pragma warning disable 612, 618
            modelBuilder
                .HasAnnotation("ProductVersion", "6.0.12")
                .HasAnnotation("Relational:MaxIdentifierLength", 64);

            modelBuilder.Entity("Aoba.Database.Models.FileData", b =>
                {
                    b.Property<ulong>("Id")
                        .ValueGeneratedOnAdd()
                        .HasColumnType("bigint unsigned");

                    b.Property<string>("Content")
                        .IsRequired()
                        .HasColumnType("LONGTEXT");

                    b.Property<string>("MimeType")
                        .IsRequired()
                        .HasMaxLength(255)
                        .HasColumnType("varchar(255)");

                    b.HasKey("Id");

                    b.ToTable("FilesData", (string)null);
                });

            modelBuilder.Entity("Aoba.Database.Models.FileMetadata", b =>
                {
                    b.Property<string>("Id")
                        .HasMaxLength(100)
                        .HasColumnType("varchar(100)");

                    b.Property<long>("CreatedAt")
                        .HasColumnType("bigint");

                    b.Property<ulong>("DataId")
                        .HasMaxLength(100)
                        .HasColumnType("bigint unsigned");

                    b.Property<bool>("IsDirectory")
                        .HasColumnType("tinyint(1)");

                    b.Property<bool>("IsListed")
                        .HasColumnType("tinyint(1)");

                    b.Property<bool>("IsPublic")
                        .HasColumnType("tinyint(1)");

                    b.Property<string>("Name")
                        .IsRequired()
                        .HasMaxLength(255)
                        .HasColumnType("varchar(255)");

                    b.Property<string>("OwnerId")
                        .IsRequired()
                        .HasMaxLength(100)
                        .HasColumnType("varchar(100)");

                    b.Property<string>("ParentId")
                        .HasMaxLength(100)
                        .HasColumnType("varchar(100)");

                    b.HasKey("Id");

                    b.HasIndex("DataId")
                        .IsUnique();

                    b.HasIndex("OwnerId");

                    b.HasIndex("ParentId");

                    b.ToTable("Files", (string)null);
                });

            modelBuilder.Entity("Aoba.Database.Models.User", b =>
                {
                    b.Property<string>("Id")
                        .HasMaxLength(100)
                        .HasColumnType("varchar(100)");

                    b.Property<string>("LastToken")
                        .HasColumnType("longtext");

                    b.Property<string>("Password")
                        .IsRequired()
                        .HasMaxLength(100)
                        .HasColumnType("varchar(100)");

                    b.Property<string>("Username")
                        .IsRequired()
                        .HasMaxLength(255)
                        .HasColumnType("varchar(255)");

                    b.HasKey("Id");

                    b.HasIndex("Username")
                        .IsUnique();

                    b.ToTable("Users", (string)null);
                });

            modelBuilder.Entity("Aoba.Database.Models.FileMetadata", b =>
                {
                    b.HasOne("Aoba.Database.Models.FileData", "Data")
                        .WithOne("File")
                        .HasForeignKey("Aoba.Database.Models.FileMetadata", "DataId")
                        .OnDelete(DeleteBehavior.Cascade)
                        .IsRequired();

                    b.HasOne("Aoba.Database.Models.User", "Owner")
                        .WithMany("Files")
                        .HasForeignKey("OwnerId")
                        .OnDelete(DeleteBehavior.Cascade)
                        .IsRequired();

                    b.HasOne("Aoba.Database.Models.FileMetadata", "Parent")
                        .WithMany("Children")
                        .HasForeignKey("ParentId");

                    b.Navigation("Data");

                    b.Navigation("Owner");

                    b.Navigation("Parent");
                });

            modelBuilder.Entity("Aoba.Database.Models.FileData", b =>
                {
                    b.Navigation("File");
                });

            modelBuilder.Entity("Aoba.Database.Models.FileMetadata", b =>
                {
                    b.Navigation("Children");
                });

            modelBuilder.Entity("Aoba.Database.Models.User", b =>
                {
                    b.Navigation("Files");
                });
#pragma warning restore 612, 618
        }
    }
}
