import { Token, Tokenize } from '../components/tokenize';

// @author Alvin Wong
// Tests the methods in tokenize.tsx
// Each test is labeled with the specific functionality it checks for.
describe('Tokenize Tests', () => {
    test('Tokenizes references correctly', () => {
        const tokenizer = new Tokenize('$A1 $B2\n$C3');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'REF', value: '$A1' },
            { type: 'REF', value: '$B2' },
            { type: 'REF', value: '$C3' },
            { type: 'FORMULA', value: '' },
        ]);
    });

    test('Tokenizes formulas correctly', () => {
        const tokenizer = new Tokenize('=SUM($A1, $B2)');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'OPERATOR', value: '=' },
            { type: 'FUNCTION', value: 'SUM' },
            { type: 'OPERATOR', value: '(' },
            { type: 'REF', value: '$A1' },
            { type: 'OPERATOR', value: ',' },
            { type: 'REF', value: '$B2' },
            { type: 'OPERATOR', value: ')' },
            { type: 'FORMULA', value: '' },
        ]);
    });

    test('Tokenizes operators correctly', () => {
        const tokenizer = new Tokenize('+ - * / < > = <> & | :');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'OPERATOR', value: '+' },
            { type: 'OPERATOR', value: '-' },
            { type: 'OPERATOR', value: '*' },
            { type: 'OPERATOR', value: '/' },
            { type: 'OPERATOR', value: '<' },
            { type: 'OPERATOR', value: '>' },
            { type: 'OPERATOR', value: '=' },
            { type: 'OPERATOR', value: '<>' },
            { type: 'OPERATOR', value: '&' },
            { type: 'OPERATOR', value: '|' },
            { type: 'OPERATOR', value: ':' },
            { type: 'FORMULA', value: '' },
        ]);
    });

    test('Tokenizes numbers correctly', () => {
        const tokenizer = new Tokenize('1 23 45.67 890');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'NUMBER', value: '1' },
            { type: 'NUMBER', value: '23' },
            { type: 'NUMBER', value: '45.67' },
            { type: 'NUMBER', value: '890' },
            { type: 'FORMULA', value: '' },
        ]);
    });

    test('Ignores whitespace', () => {
        const tokenizer = new Tokenize('  \t\n $A1 123 +  ');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'REF', value: '$A1' },
            { type: 'NUMBER', value: '123' },
            { type: 'OPERATOR', value: '+' },
            { type: 'FORMULA', value: '' },
        ]);
    });

    test('Tokenizes mixed input correctly', () => {
        const tokenizer = new Tokenize('=SUM($A1, 123) > 45.67');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'OPERATOR', value: '=' },
            { type: 'FUNCTION', value: 'SUM' },
            { type: 'OPERATOR', value: '(' },
            { type: 'REF', value: '$A1' },
            { type: 'OPERATOR', value: ',' },
            { type: 'NUMBER', value: '123' },
            { type: 'OPERATOR', value: ')' },
            { type: 'OPERATOR', value: '>' },
            { type: 'NUMBER', value: '45.67' },
            { type: 'FORMULA', value: '' },
        ]);
    });

    test('Tokenizes strings correctly', () => {
        const tokenizer = new Tokenize('"Hello World"');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'CHARACTERS', value: 'Hello World' },
            { type: 'FORMULA', value: '' },
        ]);
    });

    test('Tokenizes complex formulas correctly', () => {
        const tokenizer = new Tokenize('=IF($A1 > 10, "Pass", "Fail")');
        expect(tokenizer.tokenize()).toEqual([
            { type: 'OPERATOR', value: '=' },
            { type: 'FUNCTION', value: 'IF' },
            { type: 'OPERATOR', value: '(' },
            { type: 'REF', value: '$A1' },
            { type: 'OPERATOR', value: '>' },
            { type: 'NUMBER', value: '10' },
            { type: 'OPERATOR', value: ',' },
            { type: 'CHARACTERS', value: 'Pass' },
            { type: 'OPERATOR', value: ',' },
            { type: 'CHARACTERS', value: 'Fail' },
            { type: 'OPERATOR', value: ')' },
            { type: 'FORMULA', value: '' },
        ]);
    });
});
