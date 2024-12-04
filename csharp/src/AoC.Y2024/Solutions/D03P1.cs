using System.Text.RegularExpressions;

namespace AoC.Y2024.Solutions;

public partial class D03P1 : ISolution
{
    [GeneratedRegex(@"mul\((\d+),(\d+)\)", RegexOptions.Compiled)]
    private static partial Regex Parser { get; }

    public Task Run(SolutionContext context)
    {
        var sum = Parser
            .Matches(context.InputString)
            .Select(m => m.Groups[1].Value.ConvertTo<int>() * m.Groups[2].Value.ConvertTo<int>())
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}
