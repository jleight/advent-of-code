namespace AoC.Y2022.Solutions;

[UsedImplicitly]
public class D07P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var root = Directory.MakeRoot();
        var current = root;

        var directories = new Dictionary<string, Directory>
        {
            [root.Path] = root
        };

        for (var i = 0; i < context.InputLines.Length; i++)
        {
            var line = context.InputLines[i];

            if (line == "$ cd /")
            {
                current = root;
                continue;
            }

            if (line == "$ cd ..")
            {
                current = current.Parent;
                continue;
            }

            if (line.StartsWith("$ cd "))
            {
                current = current.Directories[line[5..]];
                continue;
            }

            if (line == "$ ls")
            {
                var records = context.InputLines
                    .Skip(i + 1)
                    .TakeWhile(l => !l.StartsWith("$"))
                    .ToList();

                i += records.Count;

                foreach (var record in records)
                {
                    if (record.StartsWith("dir "))
                    {
                        var name = record[4..];
                        if (current.Directories.ContainsKey(name))
                            continue;

                        var newDirectory = new Directory(current, name);
                        current.Directories[name] = newDirectory;
                        directories[newDirectory.Path] = newDirectory;
                    }
                    else
                    {
                        var split = record.Split(" ");
                        current.Files[split[1]] = int.Parse(split[0]);
                    }
                }

                continue;
            }

            Console.Error.WriteLine("Found invalid command: {line}");
        }

        var result = directories
            .Values
            .Select(d => d.Size)
            .Where(s => s <= 100000)
            .Sum();

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}

file class Directory
{
    public Directory Parent { get; }

    public Dictionary<string, Directory> Directories { get; } = new();
    public Dictionary<string, int> Files { get; } = new();

    public string Path { get; }
    public int Size => Directories.Values.Sum(d => d.Size) + Files.Values.Sum();

    private Directory()
    {
        Parent = this;
        Path = "/";
    }

    public Directory(Directory parent, string name)
    {
        Parent = parent;
        Path = $"{Parent.Path.TrimEnd('/')}/{name}";
    }

    public static Directory MakeRoot()
        => new();
}
