namespace AoC.Utils;

public record Point<TMetadata>(
    int X,
    int Y,
    TMetadata Metadata);

public static class Point
{
    public static Point<TMetadata> New<TMetadata>(int x, int y, TMetadata metadata)
        => new(x, y, metadata);
}
