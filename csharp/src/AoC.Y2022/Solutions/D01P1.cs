namespace AoC.Y2022.Solutions;

public class D01P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines.Append(string.Empty);
        var max = 0;
        var sum = 0;

        foreach (var line in input)
        {
            max = Math.Max(max, sum);
            sum += line.Length == 0 ? -sum : int.Parse(line);
        }

        Console.WriteLine(max);
        return Task.CompletedTask;
    }
}
