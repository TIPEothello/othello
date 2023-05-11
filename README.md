# Othello

## Rules

Othello is a game of strategy played between two players on an 8x8 board. The game is played with black and white pieces. The goal of the game is to have the majority of pieces on the board at the end of the game. The game is played by placing pieces on the board, and then flipping the pieces of the opponent that are in between the placed piece and another piece of the same color. The game ends when neither player can place a piece on the board.

## First approach : Minimax

The first approach to this problem was to use the Minimax algorithm. This algorithm is a recursive algorithm that is used to choose an optimal move for a player assuming that the opponent is also playing optimally. The algorithm works by generating a tree of all possible moves that can be made by both players. The algorithm then evaluates each node in the tree with a heuristic function. The heuristic function is used to determine the value of a node. The algorithm then chooses the move that leads to the node with the highest value. The algorithm is recursive because it calls itself to evaluate the nodes in the tree. The algorithm also uses alpha-beta pruning to reduce the number of nodes that need to be evaluated. Alpha-beta pruning is a technique that is used to reduce the number of nodes that need to be evaluated by the algorithm. The algorithm works by keeping track of the best possible move that can be made by the maximizing player and the best possible move that can be made by the minimizing player. The algorithm then prunes all nodes that are worse than the best possible move for the maximizing player and all nodes that are better than the best possible move for the minimizing player. The algorithm then returns the best possible move for the maximizing player.

## Problems with Minimax

