namespace AoC.Y2022.Solutions;

[UsedImplicitly]
public class D02P1 : ISolution
{
    private static readonly Dictionary<string, int> _pointsByCombination = new()
    {
        ["A X"] = 3 + 1,
        ["A Y"] = 6 + 2,
        ["A Z"] = 0 + 3,
        ["B X"] = 0 + 1,
        ["B Y"] = 3 + 2,
        ["B Z"] = 6 + 3,
        ["C X"] = 6 + 1,
        ["C Y"] = 0 + 2,
        ["C Z"] = 3 + 3
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
