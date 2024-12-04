using System.Text.RegularExpressions;

namespace AoC.Y2024.Solutions;

public partial class D03P2 : ISolution
{
    [GeneratedRegex(@"don't\(\)(?:.+?do\(\)|$)", RegexOptions.Compiled)]
    private static partial Regex Cleaner { get; }

    [GeneratedRegex(@"mul\((\d+),(\d+)\)", RegexOptions.Compiled)]
    private static partial Regex Parser { get; }

    public Task Run(SolutionContext context)
    {
        var cleaned = Cleaner.Replace(
            string.Join(string.Empty, context.InputLines),
            string.Empty);

        var sum = Parser
            .Matches(cleaned)
            .Select(m => m.Groups[1].Value.ConvertTo<int>() * m.Groups[2].Value.ConvertTo<int>())
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}
