namespace AoC.Y2023.Solutions;

public class D04P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var points = context
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
            .Where(x => x.Matches > 0)
            .Select(x => Math.Pow(2, x.Matches - 1))
            .Sum();

        Console.WriteLine(points);
        return Task.CompletedTask;
    }
}
