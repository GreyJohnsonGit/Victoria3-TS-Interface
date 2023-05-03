using DotNext;

namespace Vicky {
  public static class OptionalExtension {
    public static Optional<R> Map<T, R>(
      this Optional<T> optional, 
      Func<T, R> map
    ) {
      if (optional.HasValue)
        return map(optional.Value);
      else
        return new Optional<R>();
    }

    public static Optional<T> Then<T>(
      this Optional<T> optional, 
      Action<T> action
    ) {
      if (optional.HasValue)
        action(optional.Value);

      return optional;
    }

    public static bool IsNone<T>(this Optional<T> optional) {
      return !optional.HasValue;
    }

    public static bool IsSome<T>(this Optional<T> optional) {
      return optional.HasValue;
    }
  }
}