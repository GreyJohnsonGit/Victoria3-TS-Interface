using Optional;

namespace Vicky {
  public static class EnumExtensions {
    public static Option<T> Into<T>(this string value) where T : struct, Enum {
      if (Enum.TryParse<T>(value, out var result)) {
        return result.Some();
      } else {
        return Option.None<T>();
      }
    }
  }

  public static class NullableExtensions {
    public static Option<R> IfNotNull<T, R>(this T? value, Func<T, R> onNotNull) {
      if (value == null) {
        return Option.None<R>();
      } else {
        return onNotNull(value).Some();
      }
    }
  }
}