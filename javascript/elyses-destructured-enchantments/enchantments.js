/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Get the first card in the given deck
 *
 * @param {Card[]} deck
 *
 * @returns {Card} the first card in the deck
 */
export function getFirstCard([first]) {
  return first;
}

/**
 * Get the second card in the given deck
 *
 * @param {Card[]} deck
 *
 * @returns {Card} the second card in the deck
 */
export function getSecondCard([, second]) {
  return second;
}

/**
 * Switch the position of the two cards in the given deck
 *
 * @param {Card[]} deck
 *
 * @returns {Card[]} new deck with reordered cards
 */
export function swapTwoCards([first, second]) {
  return [second, first];
}

/**
 * Put the first card of the given deck into the back of the deck.
 *
 * @param {Card[]} deck
 *
 * @returns {Card[]} deck
 */
export function shiftThreeCardsAround([first, second, third]) {
  return [second, third, first];
}

/**
 * @param {{chosen: Card[], disregarded: Card[]}} deck
 *
 * @returns {Card[]} chosen
 */
export function pickNamedPile({ chosen }) {
  return chosen;
}

/**
 * @param {{chosen: Card[], disregarded: Card[]}}
 *
 * @returns {{chosen: Card[], disregarded: Card[]}}
 */
export function swapNamedPile({ chosen, disregarded }) {
  return { chosen: disregarded, disregarded: chosen };
}
