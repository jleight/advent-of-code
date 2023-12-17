namespace AoC.Y2023.Solutions;

public class D15P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = context.InputLines
            .SelectMany(l => l.Split(','))
            .Select(Hash)
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    private static int Hash(string input)
    {
        var current = 0;

        foreach (var c in input)
        {
            current += c;
            current *= 17;
            current %= 256;
        }

        return current;
    }
}
