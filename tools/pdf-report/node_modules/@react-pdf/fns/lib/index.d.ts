/**
 * Applies a function to the value at the given index of an array
 *
 * @param index
 * @param fn
 * @param collection
 * @returns Copy of the array with the element at the given index replaced with the result of the function application.
 */
declare const adjust: <T>(index: number, fn: (value: T) => T, collection: T[]) => T[];

type Fn$1 = (arg: any, ...args: any[]) => Promise<any> | any;
type ComposedInput$1<T extends Fn$1[]> = T extends [
    ...any,
    (arg: infer A, ...args: any[]) => Promise<any> | any
] ? A : never;
type ComposedOutput$1<T extends Fn$1[]> = T extends [
    (arg: any, ...args: any[]) => Promise<infer R> | infer R,
    ...any
] ? R : never;
/**
 * Performs right-to-left function composition with async functions support.
 * asyncCompose(f, g, h)(x) is equivalent to f(g(h(x))), awaiting each result.
 *
 * @param fns - Functions to compose (can be sync or async)
 * @returns Composed async function that applies functions from right to left
 */
declare const asyncCompose: <T extends Fn$1[]>(...fns: T) => (value: ComposedInput$1<T>, ...args: Parameters<T[0]> extends [any, ...infer Rest] ? Rest : []) => Promise<ComposedOutput$1<T>>;

/**
 * Capitalize first letter of each word
 *
 * @param value - Any string
 * @returns Capitalized string
 */
declare const capitalize: (value?: string | null) => string | null | undefined;

/**
 * Casts value to array
 *
 * @template T - The type of the value.
 * @param value - The value to cast into an array.
 * @returns The value as-is if already an array, otherwise wrapped in an array.
 */
declare const castArray: <T>(value: T | T[]) => T[];

type Fn = (arg: any, ...args: any[]) => any;
type ComposedInput<T extends Fn[]> = T extends [
    ...any,
    (arg: infer A, ...args: any[]) => any
] ? A : never;
type ComposedOutput<T extends Fn[]> = T extends [
    (arg: any, ...args: any[]) => infer R,
    ...any
] ? R : never;
/**
 * Performs right-to-left function composition.
 * compose(f, g, h)(x) is equivalent to f(g(h(x)))
 *
 * @param fns - Functions to compose
 * @returns Composed function that applies functions from right to left
 */
declare const compose: <T extends Fn[]>(...fns: T) => (value: ComposedInput<T>, ...args: any[]) => ComposedOutput<T>;

/**
 * Drops the last element from an array or string.
 *
 * @param value - The array or string to drop the last element from
 * @returns A new array or string with the last element removed
 */
declare function dropLast(value: string): string;
declare function dropLast<T>(value: T[]): T[];

/**
 * Applies a set of transformations to an object and returns a new object with the transformed values.
 *
 * @example
 * evolve({ count: (n) => n + 1 }, { name: 'item', count: 5 })
 * // => { name: 'item', count: 6 }
 *
 * @param transformations - The transformations to apply
 * @param object - The object to transform
 * @returns The transformed object
 */
declare function evolve<T extends Record<string, any>>(transformations: Partial<{
    [K in keyof T]: (value: T[K]) => T[K];
}>, object: T): T;

/**
 * Retrieves the value at a given path from an object.
 *
 * @example
 * get({ a: { b: 1 } }, ['a', 'b'], 0) // => 1
 * get({ a: { b: 1 } }, ['a', 'c'], 0) // => 0
 *
 * @param target - The object to retrieve the value from
 * @param path - The path of the value to retrieve
 * @param defaultValue - The default value to return if the path does not exist
 * @returns The value at the given path, or the default value if the path does not exist
 */
declare const get: (target: any, path: (string | number)[] | string | number, defaultValue: any) => any;

/**
 * Checks if a value is null or undefined.
 *
 * @example
 * isNil(null)      // => true
 * isNil(undefined) // => true
 * isNil(0)         // => false
 *
 * @param value - The value to check
 * @returns True if the value is null or undefined, false otherwise
 */
declare const isNil: (value: unknown) => value is null | undefined;

/**
 * Returns the last element of an array or last character of a string.
 *
 * @example
 * last([1, 2, 3]) // => 3
 * last('abc')     // => 'c'
 * last([])        // => undefined
 * last('')        // => ''
 *
 * @param value - The array or string
 * @returns The last element/character, or undefined for empty arrays
 */
declare function last(value: string): string;
declare function last<T>(value: T[]): T | undefined;

type IteratorFn = (value: any, key: string, index: number) => any;
/**
 * Maps over the values of an object and applies a function to each value.
 *
 * @example
 * mapValues({ a: 1, b: 2 }, (v) => v * 2) // => { a: 2, b: 4 }
 *
 * @param object - The object to map over
 * @param fn - The function to apply to each value
 * @returns A new object with the mapped values
 */
declare const mapValues: (object: Record<string, any>, fn: IteratorFn) => Record<string, any>;

interface PercentMatch {
    percent: number;
    value: number;
}
/**
 * Parses a percentage string and returns both the numeric value and decimal percent.
 *
 * @example
 * matchPercent('50%')  // => { value: 50, percent: 0.5 }
 * matchPercent('-25%') // => { value: -25, percent: -0.25 }
 * matchPercent('abc')  // => null
 *
 * @param value - The value to parse
 * @returns Object with value and percent, or null if not a valid percentage
 */
declare const matchPercent: (value: string | number | null) => PercentMatch | null;

/**
 * Creates a new object by omitting specified keys from the original object.
 *
 * @example
 * omit('b', { a: 1, b: 2, c: 3 })      // => { a: 1, c: 3 }
 * omit(['a', 'c'], { a: 1, b: 2, c: 3 }) // => { b: 2 }
 *
 * @param keys - The key or keys to omit
 * @param object - The original object
 * @returns The new object without the omitted keys
 */
declare const omit: (keys: string | string[], object: Record<string, any>) => Record<string, any>;

/**
 * Picks the specified keys from an object and returns a new object with only those keys.
 *
 * @example
 * pick(['a', 'c'], { a: 1, b: 2, c: 3 }) // => { a: 1, c: 3 }
 * pick(['x'], { a: 1, b: 2 })            // => {}
 *
 * @param keys - The keys to pick from the object
 * @param object - The object to pick the keys from
 * @returns A new object with only the picked keys
 */
declare const pick: (keys: (string | number)[], object: Record<string, any>) => Record<string, any>;

/**
 * Repeats an element a specified number of times.
 *
 * @example
 * repeat('a', 3) // => ['a', 'a', 'a']
 * repeat(0, 4)   // => [0, 0, 0, 0]
 * repeat('x')    // => []
 *
 * @param element - Element to be repeated
 * @param length - Number of times to repeat element (default: 0)
 * @returns Array with the element repeated
 */
declare const repeat: <T>(element: T, length?: number) => T[];

/**
 * Returns a new array with elements in reverse order. Does not mutate the original.
 *
 * @example
 * reverse([1, 2, 3]) // => [3, 2, 1]
 * reverse(['a', 'b']) // => ['b', 'a']
 *
 * @param array - Array to be reversed
 * @returns New array with elements reversed
 */
declare const reverse: <T>(array: T[]) => T[];

/**
 * Converts the first character of a string to uppercase. Does not affect other characters.
 *
 * @example
 * upperFirst('hello')  // => 'Hello'
 * upperFirst('hELLO')  // => 'HELLO'
 * upperFirst('')       // => ''
 * upperFirst(null)     // => null
 *
 * @param value - The string to transform
 * @returns String with first character uppercased, or the original value if null/undefined/empty
 */
declare const upperFirst: <T extends string | null | undefined>(value: T) => T extends string ? string : T;

/**
 * Returns a new array excluding the specified values.
 *
 * @example
 * without([2, 4], [1, 2, 3, 4, 5]) // => [1, 3, 5]
 * without(['b'], ['a', 'b', 'c'])  // => ['a', 'c']
 *
 * @param exclude - Values to exclude from the array
 * @param array - Array to filter
 * @returns A new array without the excluded values
 */
declare const without: <T>(exclude: T[], array: T[]) => T[];

/**
 * Parse a string or number to a float. Non-string values pass through unchanged.
 *
 * @example
 * parseFloat('3.14')  // => 3.14
 * parseFloat(42)      // => 42
 * parseFloat('10px')  // => 10
 * parseFloat(null)    // => null
 *
 * @param value - The value to parse
 * @returns Parsed float for strings, original value otherwise
 */
declare const parseFloat: <T extends string | number | null | undefined>(value: T) => T extends string ? number : T;

export { adjust, asyncCompose, capitalize, castArray, compose, dropLast, evolve, get, isNil, last, mapValues, matchPercent, omit, parseFloat, pick, repeat, reverse, upperFirst, without };
