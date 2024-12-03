using System.Threading.Channels;

namespace AoC.Y2024.Solutions;

public class D01P1 : ISolution
{
    public async Task Run(SolutionContext context)
    {
        var left = Channel.CreateUnboundedPrioritized<int>();
        var right = Channel.CreateUnboundedPrioritized<int>();

        foreach (var line in context.InputLines)
        {
            var parts = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);

            await left.Writer.WriteAsync(int.Parse(parts.First()));
            await right.Writer.WriteAsync(int.Parse(parts.Last()));
        }

        left.Writer.Complete();
        right.Writer.Complete();

        var sum = 0;

        await foreach (var (l, r) in left.Zip(right))
            sum += Math.Abs(l - r);

        Console.WriteLine(sum);
    }
}
