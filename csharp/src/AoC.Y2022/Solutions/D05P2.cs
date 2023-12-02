using System.Text.RegularExpressions;

namespace AoC.Y2022.Solutions;

public partial class D05P2 : ISolution
{
    private static readonly Regex _instructionFormat = MyRegex();

    public Task Run(SolutionContext context)
    {
        var setupLines = context.InputLines
            .TakeWhile(l => l.Length > 0)
            .ToList();

        var stackLines = setupLines.Take(setupLines.Count - 1);
        var stackNumbers = setupLines.Last();
        var instructionLines = context.InputLines.SkipWhile(l => l.Length > 1).Skip(1);

        var stackCount = stackNumbers
            .Split(" ", StringSplitOptions.RemoveEmptyEntries)
            .Select(int.Parse)
            .Last();
        var stacks = Enumerable
            .Range(0, stackCount)
            .Select(_ => new Stack<char>())
            .ToList();

        foreach (var line in stackLines.Reverse())
        {
            for (var i = 0; i < stackCount; i++)
            {
                var letter = line[i * 4 + 1];
                if (letter != ' ')
                    stacks[i].Push(letter);
            }
        }

        foreach (var line in instructionLines)
        {
            var parsed = _instructionFormat.Match(line);
            var count = int.Parse(parsed.Groups[1].Value);
            var source = int.Parse(parsed.Groups[2].Value) - 1;
            var destination = int.Parse(parsed.Groups[3].Value) - 1;

            var temp = new Stack<char>();
            for (var i = 0; i < count; i++)
                temp.Push(stacks[source].Pop());
            for (var i = 0; i < count; i++)
                stacks[destination].Push(temp.Pop());
        }

        var result = new string(stacks.Select(s => s.Pop()).ToArray());

        Console.WriteLine(result);
        return Task.CompletedTask;
    }

    [GeneratedRegex("move (\\d+) from (\\d+) to (\\d+)")]
    private static partial Regex MyRegex();
}
