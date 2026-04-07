/**
 * Applies a function to the value at the given index of an array
 *
 * @param index
 * @param fn
 * @param collection
 * @returns Copy of the array with the element at the given index replaced with the result of the function application.
 */
const adjust = (index, fn, collection) => {
    if (index >= collection.length)
        return collection;
    if (index < 0 && Math.abs(index) > collection.length)
        return collection;
    const i = index < 0 ? collection.length + index : index;
    return Object.assign([], collection, { [i]: fn(collection[i]) });
};

/* eslint-disable no-await-in-loop */
/**
 * Performs right-to-left function composition with async functions support.
 * asyncCompose(f, g, h)(x) is equivalent to f(g(h(x))), awaiting each result.
 *
 * @param fns - Functions to compose (can be sync or async)
 * @returns Composed async function that applies functions from right to left
 */
const asyncCompose = (...fns) => async (value, ...args) => {
    let result = value;
    for (let i = fns.length - 1; i >= 0; i -= 1) {
        result = await fns[i](result, ...args);
    }
    return result;
};

/**
 * Capitalize first letter of each word
 *
 * @param value - Any string
 * @returns Capitalized string
 */
const capitalize = (value) => {
    if (!value)
        return value;
    return value.replace(/(^|\s)\S/g, (l) => l.toUpperCase());
};

/**
 * Casts value to array
 *
 * @template T - The type of the value.
 * @param value - The value to cast into an array.
 * @returns The value as-is if already an array, otherwise wrapped in an array.
 */
const castArray = (value) => {
    return Array.isArray(value) ? value : [value];
};

/**
 * Performs right-to-left function composition.
 * compose(f, g, h)(x) is equivalent to f(g(h(x)))
 *
 * @param fns - Functions to compose
 * @returns Composed function that applies functions from right to left
 */
const compose = (...fns) => (value, ...args) => {
    let result = value;
    for (let i = fns.length - 1; i >= 0; i -= 1) {
        result = fns[i](result, ...args);
    }
    return result;
};

function dropLast(value) {
    return value.slice(0, -1);
}

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
function evolve(transformations, object) {
    const result = Object.create(null);
    const keys = Object.keys(object);
    for (let i = 0; i < keys.length; i += 1) {
        const key = keys[i];
        if (key === '__proto__' || key === 'constructor' || key === 'prototype') {
            continue;
        }
        const transformation = transformations[key];
        if (typeof transformation === 'function') {
            result[key] = transformation(object[key]);
        }
        else {
            result[key] = object[key];
        }
    }
    return result;
}

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
const isNil = (value) => value === null || value === undefined;

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
const get = (target, path, defaultValue) => {
    if (isNil(target))
        return defaultValue;
    const _path = castArray(path);
    let result = target;
    for (let i = 0; i < _path.length; i += 1) {
        if (isNil(result))
            return defaultValue;
        result = result[_path[i]];
    }
    return isNil(result) ? defaultValue : result;
};

function last(value) {
    return value === '' ? '' : value[value.length - 1];
}

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
const mapValues = (object, fn) => {
    const result = {};
    const entries = Object.entries(object);
    for (let i = 0; i < entries.length; i += 1) {
        const [key, value] = entries[i];
        if (key === '__proto__' || key === 'constructor' || key === 'prototype') {
            continue;
        }
        result[key] = fn(value, key, i);
    }
    return result;
};

const PERCENT_REGEX = /(-?\d+\.?\d*)%/;
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
const matchPercent = (value) => {
    const match = PERCENT_REGEX.exec(`${value}`);
    if (match) {
        const numericValue = parseFloat(match[1]);
        const percent = numericValue / 100;
        return { percent, value: numericValue };
    }
    return null;
};

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
const omit = (keys, object) => {
    const _keys = castArray(keys);
    const copy = { ...object };
    for (let i = 0; i < _keys.length; i += 1) {
        delete copy[_keys[i]];
    }
    return copy;
};

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
const pick = (keys, object) => {
    const result = {};
    for (let i = 0; i < keys.length; i += 1) {
        const key = keys[i];
        if (key === '__proto__' || key === 'constructor' || key === 'prototype') {
            continue;
        }
        if (key in object)
            result[key] = object[key];
    }
    return result;
};

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
const repeat = (element, length = 0) => {
    const result = new Array(length);
    for (let i = 0; i < length; i += 1) {
        result[i] = element;
    }
    return result;
};

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
const reverse = (array) => array.slice().reverse();

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
const upperFirst = (value) => {
    if (!value)
        return value;
    return (value.charAt(0).toUpperCase() + value.slice(1));
};

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
const without = (exclude, array) => {
    const result = [];
    for (let i = 0; i < array.length; i += 1) {
        const value = array[i];
        if (!exclude.includes(value))
            result.push(value);
    }
    return result;
};

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
const parseFloat$1 = (value) => {
    return (typeof value === 'string' ? Number.parseFloat(value) : value);
};

export { adjust, asyncCompose, capitalize, castArray, compose, dropLast, evolve, get, isNil, last, mapValues, matchPercent, omit, parseFloat$1 as parseFloat, pick, repeat, reverse, upperFirst, without };
