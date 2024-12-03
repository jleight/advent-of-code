namespace AoC.Y2024.Solutions;

public class D01P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var split = context
            .InputLines
            .Select(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries))
            .Select(l => (Left: int.Parse(l[0]), Right: int.Parse(l[1])))
            .ToList();

        var histogram = split
            .GroupBy(l => l.Right)
            .ToDictionary(
                g => g.Key,
                g => g.Count());

        var sum = split
            .Select(l => l.Left * histogram.GetValueOrDefault(l.Left, 0))
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}
