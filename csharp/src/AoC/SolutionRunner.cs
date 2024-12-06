using System.Reflection;
using CommandLine;
using StackExchange.Profiling;

namespace AoC;

public static class SolutionRunner
{
    public static async Task Run(
        IEnumerable<string> args)
    {
        await Parser.Default
            .ParseArguments<SolutionRunnerArguments>(args)
            .MapResult(Run, _ => Task.FromResult(-1));
    }

    private static async Task Run(
        SolutionRunnerArguments args)
    {
        var assembly = Assembly.GetEntryAssembly() ??
                       throw new InvalidOperationException("No entry Assembly found!");

        var solutionTypes = assembly
            .DefinedTypes
            .Where(t => typeof(ISolution).IsAssignableFrom(t))
            .Where(t => t is { IsAbstract: false, IsInterface: false, IsPublic: true })
            .OrderByDescending(t => t.Name);

        var solutionType = string.IsNullOrWhiteSpace(args.Problem)
            ? solutionTypes.FirstOrDefault()
            : solutionTypes.FirstOrDefault(t => t.Name.Equals(args.Problem, StringComparison.OrdinalIgnoreCase));

        if (solutionType is null)
        {
            await Console.Error.WriteLineAsync("Solution not implemented!");
            return;
        }

        if (Activator.CreateInstance(solutionType) is not ISolution solution)
        {
            await Console.Error.WriteLineAsync("Solution not an ISolution!");
            return;
        }

        if (string.IsNullOrWhiteSpace(args.InputsPath))
        {
            var year = Assembly
                .GetEntryAssembly()!
                .GetName()
                .Name!
                .Split(".")
                .Last()[1..];

            args.InputsPath = Directory.GetCurrentDirectory();

            while (args.InputsPath is not null)
            {
                var gitPath = Path.Combine(
                    args.InputsPath,
                    ".git");

                if (Directory.Exists(gitPath))
                {
                    args.InputsPath = Path.Combine(
                        args.InputsPath,
                        "inputs",
                        year);
                    break;
                }

                args.InputsPath = Directory
                    .GetParent(args.InputsPath)?
                    .FullName;
            }
        }

        if (string.IsNullOrWhiteSpace(args.InputsPath) ||
            !Directory.Exists(args.InputsPath))
        {
            await Console.Error.WriteLineAsync("Could not find inputs folder!");
            return;
        }

        var context = await CreateSolutionContext(
            solutionType.Name,
            args.InputsPath,
            args.UseTestData);

        var profiler = MiniProfiler.StartNew(solutionType.Name);

        using (profiler.Step("Run"))
            await solution.Run(context);

        Console.WriteLine();
        Console.WriteLine(profiler.RenderPlainText());
    }

    private static async Task<SolutionContext> CreateSolutionContext(
        string problem,
        string inputsPath,
        bool useTestData)
    {
        var suffix = useTestData
            ? "_Test"
            : string.Empty;

        var input = GetInput(inputsPath, $"{problem}{suffix}.txt") ??
                    GetInput(inputsPath, $"{problem.Split("P").First()}{suffix}.txt") ??
                    throw new InvalidOperationException($"Could not find input for {problem}.");

        return await CreateSolutionContext(input);
    }

    private static async Task<SolutionContext> CreateSolutionContext(
        Stream input)
    {
        var inputString = await new StreamReader(input)
            .ReadToEndAsync();
        var inputLines = inputString
            .Split('\r', '\n');
        return new(inputString, inputLines);
    }

    private static Stream? GetInput(
        string inputsPath,
        string name)
    {
        var path = Path.Join(
            inputsPath,
            name);

        try
        {
            return File.OpenRead(path);
        }
        catch (FileNotFoundException)
        {
            return default;
        }
    }
}
