namespace AoC.Y2022.Solutions;

[UsedImplicitly]
public class D01P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines.Append(string.Empty);
        var sums = new List<int>();
        var sum = 0;

        foreach (var line in input)
        {
            if (line.Length == 0)
            {
                sums.Add(sum);
                sum = 0;
            }
            else
            {
                sum += int.Parse(line);
            }
        }

        Console.WriteLine(sums.OrderDescending().Take(3).Sum());
        return Task.CompletedTask;
    }
}
