namespace AoC.Y2018.Solutions;

[UsedImplicitly]
public class D05P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputString;

        var result = input
            .ToUpperInvariant()
            .Distinct()
            .ToList()
            .Select(unit => Replace(unit, input))
            .Select(D05P1.React)
            .Min(polymer => polymer.Length);

        Console.WriteLine(result);
        return Task.CompletedTask;
    }

    private static string Replace(char unit, string polymer)
    {
        return polymer
            .Replace(unit.ToString(), string.Empty)
            .Replace(char.ToLowerInvariant(unit).ToString(), string.Empty);
    }
}
