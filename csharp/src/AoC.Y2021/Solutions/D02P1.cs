namespace AoC.Y2021.Solutions;

public class D02P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(Parse)
            .ToArray();

        var position = input
            .Where(i => i.DeltaAxis == Axis.Position)
            .Sum(i => i.Delta);
        var depth = input
            .Where(i => i.DeltaAxis == Axis.Depth)
            .Sum(i => i.Delta);

        Console.WriteLine(position * depth);
        return Task.CompletedTask;
    }

    private static (Axis DeltaAxis, int Delta) Parse(string line)
    {
        var split = line.Split(" ");
        var direction = split[0].ToLowerInvariant();
        var delta = int.Parse(split[1]);

        return direction switch
        {
            "forward" => (Axis.Position, delta),
            "up" => (Axis.Depth, -delta),
            "down" => (Axis.Depth, delta),
            _ => throw new InvalidOperationException($"{direction} is an invalid direction!")
        };
    }

    private enum Axis
    {
        Position,
        Depth
    }
}
