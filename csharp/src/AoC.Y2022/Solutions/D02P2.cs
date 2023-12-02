namespace AoC.Y2022.Solutions;

public class D02P2 : ISolution
{
    private static readonly Dictionary<string, int> _pointsByCombination = new()
    {
        ["A X"] = 0 + 3,
        ["A Y"] = 3 + 1,
        ["A Z"] = 6 + 2,
        ["B X"] = 0 + 1,
        ["B Y"] = 3 + 2,
        ["B Z"] = 6 + 3,
        ["C X"] = 0 + 2,
        ["C Y"] = 3 + 3,
        ["C Z"] = 6 + 1
    };

    public Task Run(SolutionContext context)
    {
        var result = context.InputLines
            .Select(c => _pointsByCombination[c])
            .Sum();

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}
