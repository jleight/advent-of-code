namespace AoC.Y2023.Solutions;

public class D04P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var matchesPerCard = context
            .InputLines
            .Select((l, i) => new
            {
                Card = i + 1,
                Numbers = l.Split(':')[1].Split('|')
            })
            .Select(x => new
            {
                x.Card,
                WinningNumbers = x.Numbers[0]
                    .Split(" ", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                    .Select(int.Parse)
                    .ToHashSet(),
                MyNumbers = x.Numbers[1]
                    .Split(" ", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                    .Select(int.Parse)
                    .ToHashSet()
            })
            .Select(x => new
            {
                x.Card,
                Matches = x.WinningNumbers.Intersect(x.MyNumbers).Count()
            })
            .ToDictionary(
                x => x.Card,
                x => x.Matches);

        var copies = Enumerable
            .Repeat(1, matchesPerCard.Count + 1)
            .ToArray();

        foreach (var (card, matches) in matchesPerCard)
        {
            for (var i = 1; i <= matches; i++)
                copies[card + i] += copies[card];
        }

        var result = copies
            .Skip(1)
            .Sum();

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}
