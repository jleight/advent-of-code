namespace AoC.Y2024.Solutions;

public class D07P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = 0L;

        foreach (var line in context.InputLines)
        {
            var (target, numbers) = Test.Parse(line);

            var success = Enumerable
                .Range(0, (int)Math.Pow(3, numbers.Length - 1))
                .Select(i => Maths
                    .ToBase(i, '+', '*', '|')
                    .PadLeft(numbers.Length - 1, '+'))
                .Select(a => Evaluate(numbers, a))
                .Any(t => t == target);

            sum += success
                ? target
                : 0;
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    private static long Evaluate(
        int[] numbers,
        ReadOnlySpan<char> operators)
    {
        var queue = new Queue<int>(numbers);
        long total = queue.Dequeue();

        foreach (var op in operators)
        {
            total = op switch
            {
                '+' => total + queue.Dequeue(),
                '*' => total * queue.Dequeue(),
                '|' => long.Parse($"{total}{queue.Dequeue()}"),
                _ => throw new InvalidOperationException()
            };
        }

        return total;
    }
}

file sealed record Test(
    long Target,
    int[] Numbers)
{
    public static Test Parse(string line)
    {
        var parts = line.Split(':');

        return new Test(
            long.Parse(parts[0]),
            parts[1]
                .Split(' ', StringSplitOptions.RemoveEmptyEntries)
                .Select(int.Parse)
                .ToArray());
    }
}
