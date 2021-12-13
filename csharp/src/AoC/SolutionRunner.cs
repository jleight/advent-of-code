using System.Reflection;
using AoC.Contexts;
using CommandLine;
using StackExchange.Profiling;

namespace AoC;

public static class SolutionRunner
{
    public static async Task Run(IEnumerable<string> args)
    {
        await Parser.Default
            .ParseArguments<SolutionRunnerArguments>(args)
            .MapResult(Run, _ => Task.FromResult(-1));
    }

    private static async Task Run(SolutionRunnerArguments args)
    {
        var assembly = Assembly.GetEntryAssembly() ?? throw new InvalidOperationException("No entry Assembly found!");

        var solutionType = assembly
            .DefinedTypes
            .Where(t => typeof(ISolution).IsAssignableFrom(t) && !t.IsAbstract && !t.IsInterface && t.IsPublic)
            .FirstOrDefault(t => t.Name.Equals(args.Problem, StringComparison.OrdinalIgnoreCase));

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

        var profiler = MiniProfiler.StartNew(solutionType.Name);
        var context = await CreateSolutionContext(assembly, solutionType.Name, args.UseTestData);

        using (profiler.Step("Run"))
            await solution.Run(context);

        Console.WriteLine();
        Console.WriteLine(profiler.RenderPlainText());
    }

    private static async Task<SolutionContext> CreateSolutionContext(Assembly assembly, string problem, bool useTestData)
    {
        var suffix = useTestData ? "_Test" : string.Empty;

        var input = GetInput(assembly, $"{problem}{suffix}.txt") ??
                    GetInput(assembly, $"{problem.Split("P").First()}{suffix}.txt") ??
                    throw new InvalidOperationException($"Could not find input for {problem}.");

        return await CreateSolutionContext(input);
    }

    private static async Task<SolutionContext> CreateSolutionContext(Stream input)
    {
        var inputString = await new StreamReader(input)
            .ReadToEndAsync();
        var inputLines = inputString
            .Split(new[] { '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries);
        return new(inputString, inputLines);
    }

    private static Stream? GetInput(Assembly assembly, string name)
    {
        var resource = assembly
            .GetManifestResourceNames()
            .FirstOrDefault(r => r.EndsWith(name));
        if (resource is null)
            return null;

        return assembly
            .GetManifestResourceStream(resource);
    }
}
