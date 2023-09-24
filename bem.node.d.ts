export type BEMElement<TName extends string, TModifiers extends string[]> = {
	name: TName
	modifiers: TModifiers
}

export type BEMBlock<
	TName extends string,
	TElements extends BEMElement<string, string[]>[],
	TModifiers extends string[]
> = BEMElement<TName, TModifiers> & {
	elements: TElements
}

/**
 * @param bem Raw (unparsed) BEM input string.
 * @see https://crates.io/crates/bem
 * @example
 * parseBEM('foo[bar,baz]\nqux')
 * {
 *   name: 'foo',
 *   modifiers: [ 'bar', 'baz' ],
 *   elements: [ { name: 'qux', modifiers: [] } ]
 * }
 * 
 * @example
parseBEM('!')
// Uncaught TypeError: Failed to parse BEM: Pest parsing error:  --> 1:1
//   |
// 1 | !
//   | ^---
//   |
// = expected name

@throws TypeError
 */
export declare function parseBEM(
	bem: string
): BEMBlock<string, BEMElement<string, string[]>[], string[]>
