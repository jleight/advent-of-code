namespace AoC.Y2024.Solutions;

public class D05P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var rules = context
            .InputLines
            .TakeWhile(l => l.Contains('|'))
            .Select(Rule.Parse)
            .ToHashSet();

        var comparer = new Comparer(rules);

        var sum = context
            .InputLines
            .SkipWhile(l => !l.Contains(','))
            .Select(l => l.Split(',').Select(int.Parse).ToArray())
            .Select(u => new
            {
                Update = u,
                Sorted = u.OrderBy(i => i, comparer),
                Middle = u[u.Length / 2]
            })
            .Where(p => p.Update.SequenceEqual(p.Sorted))
            .Select(p => p.Middle)
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}

file sealed record Rule(
    int Before,
    int After)
{
    public static Rule Parse(string line)
    {
        var parts = line.Split('|');

        return new Rule(
            int.Parse(parts[0]),
            int.Parse(parts[1]));
    }
}

file sealed class Comparer(
    HashSet<Rule> rules)
    : IComparer<int>
{
    public int Compare(int a, int b)
    {
        if (rules.Contains(new Rule(a, b)))
            return -1;
        if (rules.Contains(new Rule(b, a)))
            return 1;
        return 0;
    }
}
