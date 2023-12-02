namespace AoC.Y2023.Solutions;

public class D01P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = context
            .InputLines
            .Select(l => l.Where(char.IsDigit).ToArray())
            .Select(l => int.Parse($"{l.First()}{l.Last()}"))
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}
