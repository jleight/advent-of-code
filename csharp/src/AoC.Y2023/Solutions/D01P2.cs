using System.Text.RegularExpressions;

namespace AoC.Y2023.Solutions;

public partial class D01P2 : ISolution
{
    private static readonly Dictionary<string, int> _numbers;
    private static readonly Regex _first;
    private static readonly Regex _last;

    static D01P2()
    {
        _numbers = new Dictionary<string, int>();

        new[] { "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" }
            .Select((w, i) => (Word: w, Number: i + 1))
            .ToList()
            .ForEach(p => _numbers[p.Word] = p.Number);

        Enumerable
            .Range(1, 9)
            .Select(i => (Word: i.ToString(), Number: i))
            .ToList()
            .ForEach(p => _numbers[p.Word] = p.Number);

        _first = GenerateFirstRegex();
        _last = GenerateLastRegex();
    }

    public Task Run(SolutionContext context)
    {
        var sum = context
            .InputLines
            .Select(l => (First: _first.Match(l).Value, Last: _last.Match(l).Value))
            .Select(p => $"{_numbers[p.First]}{_numbers[p.Last]}")
            .Select(int.Parse)
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    [GeneratedRegex("([1-9]|one|two|three|four|five|six|seven|eight|nine)", RegexOptions.Compiled)]
    private static partial Regex GenerateFirstRegex();

    [GeneratedRegex("([1-9]|one|two|three|four|five|six|seven|eight|nine)", RegexOptions.Compiled | RegexOptions.RightToLeft)]
    private static partial Regex GenerateLastRegex();
}
