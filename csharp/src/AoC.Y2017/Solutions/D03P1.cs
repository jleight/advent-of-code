namespace AoC.Y2017.Solutions;

public class D03P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = Convert.ToInt32(context.InputString);

        int LengthOfSide(int l)
            => 2 * l + 1;
        int LengthOfLayer(int l)
            => 8 * l;
        int DistanceToClosestSide(int target, params int[] values)
            => values.Select(v => Math.Abs(v - target)).OrderBy(v => v).First();

        var layer = 1;
        var bottomRight = 1;
        while (bottomRight < input)
            bottomRight += LengthOfLayer(layer++);

        layer -= 1;

        var lengthOfSide = LengthOfSide(layer) - 1;
        var halfLengthOfSide = lengthOfSide / 2;
        var distanceToClosestSide = DistanceToClosestSide(
            input,
            bottomRight - halfLengthOfSide,
            bottomRight - halfLengthOfSide - 1 * lengthOfSide,
            bottomRight - halfLengthOfSide - 2 * lengthOfSide,
            bottomRight - halfLengthOfSide - 3 * lengthOfSide);
        var distanceToCenter = distanceToClosestSide + layer;

        Console.WriteLine(distanceToCenter);
        return Task.CompletedTask;
    }
}
