#include "stdafx.h"
#include <cassert>
#include <string>
#include <iostream>
#include <fstream>
#include <sstream>
#include <queue>
#include <vector>


struct Point {
	int x;
	int y;
};

enum class EdgeType {
	top,
	right,
	bottom,
	left
};

struct Tile {
	int _tileID;
	std::vector<std::string> _tileData;
};


// A value 0-7. Values 0-3 represent the 4 different rotations. 
// If Flop&4 is set, then mirror on axis is performed as well.
typedef int Flop;

Point TransformPoint(const Point iPoint, int iWidth, Flop iFlop) {
	Point newPoint;
	int rotation = iFlop & 3;
	switch (rotation) {
	case 0:
		newPoint = iPoint;
		break;
	case 1:
		newPoint = Point{ iWidth - 1 - iPoint.y, iPoint.x };
		break;
	case 2:
		newPoint = Point{ iWidth - 1 - iPoint.x, iWidth - 1 - iPoint.y };
		break;
	case 3:
		newPoint = Point{ iPoint.y, iWidth - 1 - iPoint.x };
		break;
	}
	if (iFlop & 4) {
		newPoint = Point{ iWidth - 1 - newPoint.x, newPoint.y };
	}
	return newPoint;
}


struct TileMap {
	TileMap(const std::vector<Tile>& iTiles, int iGridWidth);
	int GetTileIndex(Point iPoint) const;
	Flop GetTileFlop(Point iPoint) const;
	void SetTile(Point iPoint, int iTileIndex, Flop iTileFlop);
	void FinishAndCalculateMap();
	int64_t CalculateCornerIDProduct() const;
	char GetBigPixel(int iX, int iY, Flop iFlop) const;

	// Init to -1, set to tile index when found tile is found for a position.
	std::vector<std::vector<int>> _tileIndexMap;
	// Init to -1, set to the flop (tile transformation) of the placed tile when found.
	std::vector<std::vector<int>> _tileFlopMap;
	
	const std::vector<Tile>& _tiles;
	int _gridWidth;
	int _startX;
	int _startY;
};


TileMap::TileMap(const std::vector<Tile>& iTiles, int iGridWidth) :
	_tiles(iTiles),
	_gridWidth(iGridWidth)
{
	_tileIndexMap = std::vector<std::vector<int>>(iGridWidth * 2 + 1, std::vector<int>(iGridWidth * 2 + 1, -1));
	_tileFlopMap = std::vector<std::vector<int>>(iGridWidth * 2 + 1, std::vector<int>(iGridWidth * 2 + 1, -1));
}

int TileMap::GetTileIndex(Point iPoint) const {
	int width = _tileIndexMap.size();
	assert(iPoint.x >= 0 && iPoint.x < width && iPoint.y >= 0 && iPoint.y < width);
	return _tileIndexMap[iPoint.y][iPoint.x];
}

Flop TileMap::GetTileFlop(Point iPoint) const {
	int width = _tileIndexMap.size();
	assert(iPoint.x >= 0 && iPoint.x < width && iPoint.y >= 0 && iPoint.y < width);
	return _tileFlopMap[iPoint.y][iPoint.x];
}

void TileMap::SetTile(Point iPoint, int iTileIndex, Flop iTileFlop) {
	_tileIndexMap[iPoint.y][iPoint.x] = iTileIndex;
	_tileFlopMap[iPoint.y][iPoint.x] = iTileFlop;
}

void TileMap::FinishAndCalculateMap() {
	_startX = 2 * _gridWidth;
	for (int x = 0; x <= _gridWidth; ++x) {
		if (this->GetTileIndex(Point{ x, _gridWidth }) != -1) {
			_startX = x;
			break;
		}
	}
	assert(_startX != 2 * _gridWidth);
	_startY = 2 * _gridWidth;
	for (int y = 0; y <= _gridWidth; ++y) {
		if (this->GetTileIndex(Point{ _gridWidth, y }) != -1) {
			_startY = y;
			break;
		}
	}
	assert(_startY != 2 * _gridWidth);
}

int64_t TileMap::CalculateCornerIDProduct() const {
	Point topLeftPoint{ _startX, _startY };
	int topLeftIndex = this->GetTileIndex(topLeftPoint);
	Point topRightPoint{ _startX + _gridWidth - 1, _startY };
	int topRightIndex = this->GetTileIndex(topRightPoint);
	Point bottomLeftPoint{ _startX, _startY + _gridWidth - 1 };
	int bottomLeftIndex = this->GetTileIndex(bottomLeftPoint);
	Point bottomRightPoint{ _startX + _gridWidth - 1, _startY + _gridWidth - 1 };
	int bottomRightIndex = this->GetTileIndex(bottomRightPoint);

	int64_t product = _tiles[this->GetTileIndex(topLeftPoint)]._tileID;
	product *= _tiles[this->GetTileIndex(topRightPoint)]._tileID;
	product *= _tiles[this->GetTileIndex(bottomLeftPoint)]._tileID;
	product *= _tiles[this->GetTileIndex(bottomRightPoint)]._tileID;
	return product;
}

char TileMap::GetBigPixel(int iX, int iY, Flop iFlop) const {
	int tileWidth = _tiles[0]._tileData.size() - 2;
	int bigWidth = _gridWidth * tileWidth;
	Point newPoint = TransformPoint(Point{ iX, iY }, bigWidth, iFlop);

	int tileX = newPoint.x / tileWidth + _startX;
	int tileXIndex = newPoint.x % tileWidth + 1;
	int tileY = newPoint.y / tileWidth + _startY;
	int tileYIndex = newPoint.y % tileWidth + 1;
	Flop tileFlop = this->GetTileFlop(Point{ tileX, tileY });

	Point floppedPoint = TransformPoint(
		Point{tileXIndex, tileYIndex},
		_tiles[0]._tileData.size(),
		tileFlop);

	int tileIndex = this->GetTileIndex(Point{ tileX, tileY });
	Tile tile = _tiles[tileIndex];

	return tile._tileData[floppedPoint.y][floppedPoint.x];
}

// A specific edge of a tile that has the flop transformation (rotations+mirror).
struct Edge {
	Edge(const Tile& iTile, EdgeType iEdgeType, Flop iFlop);
	char GetPoint(int iCoord) const;

	const Tile& _tile;
	EdgeType _edgeType;
	Flop _flop;
};

Edge::Edge(const Tile& iTile, EdgeType iEdgeType, Flop iFlop) :
	_tile(iTile), _edgeType(iEdgeType), _flop(iFlop)
{
}


char Edge::GetPoint(int iCoord) const {
	int tileWidth = this->_tile._tileData.size();
	assert(iCoord >= 0 && iCoord < tileWidth);
	Point point = [this, tileWidth](int i) {
		switch (this->_edgeType) {
		case EdgeType::top:
			return Point{ i, 0 };
			break;
		case EdgeType::right:
			return Point{ tileWidth - 1, i };
			break;
		case EdgeType::bottom:
			return Point{ i, tileWidth - 1 };
			break;
		case EdgeType::left:
			return Point{ 0, i };
			break;
		}
	}(iCoord);
	Point transformedPoint = TransformPoint(point, tileWidth, this->_flop);
	return this->_tile._tileData[transformedPoint.y][transformedPoint.x];
}

bool AreEdgesEqual(const Edge& iEdge1, const Edge& iEdge2) {
	int tileWidth = iEdge1._tile._tileData.size();
	bool equal = true;
	for (int i = 0; i < tileWidth; ++i) {
		if (iEdge1.GetPoint(i) != iEdge2.GetPoint(i)) {
			equal = false;
			break;
		}
	}
	return equal;
}


// Return tile index
bool FindTile(
	std::queue<Point>& iQueue,
	const std::vector<Tile>& iTiles,
	TileMap& iTileMap,
	Point iFrom, 
	EdgeType iDirection) 
{
	int fromTileIndex = iTileMap.GetTileIndex(iFrom);
	assert(fromTileIndex != -1);
	Flop fromTileFlop = iTileMap.GetTileFlop(iFrom);
	assert(fromTileFlop != -1);

	Tile fromTile = iTiles[fromTileIndex];
	Edge edgeFrom(fromTile, iDirection, fromTileFlop);
	
	EdgeType oppositeEdgeType = static_cast<EdgeType>((static_cast<int>(iDirection) + 2) % 4);
	static Point dirToPoint[]{ {0, -1} , {1, 0}, {0, 1}, {-1,0} };
	Point toPoint = iFrom;
	toPoint.x += dirToPoint[static_cast<int>(iDirection)].x;
	toPoint.y += dirToPoint[static_cast<int>(iDirection)].y;
	assert(toPoint.x >= 0 && toPoint.x < iTileMap._tileIndexMap.size() && toPoint.y >= 0 && toPoint.y < iTileMap._tileIndexMap.size());
	
	int toTileIndex = iTileMap.GetTileIndex(toPoint);
	if (toTileIndex == -1) {
		// Nothing place here, see if there is matching edge, and store flop
		for (int checkTileIndex = 0; checkTileIndex < iTiles.size(); ++checkTileIndex) {
			if (checkTileIndex == fromTileIndex) {
				continue;
			}
			const Tile& toTile = iTiles[checkTileIndex];
			Flop foundFlop = -1;
			for (Flop checkFlop = 0; checkFlop <= 7; ++checkFlop) {
				Edge edgeTo(toTile, oppositeEdgeType, checkFlop);
				if (AreEdgesEqual(edgeFrom, edgeTo)) {
					foundFlop = checkFlop;
					break;
				}
			}
			if (foundFlop != -1) {
				// Check that tile is not already taken.
				for (int ty = 0; ty < 24; ++ty) {
					for (int tx = 0; tx < 24; ++tx) {
						assert(iTileMap._tileIndexMap[ty][tx] != checkTileIndex);
					}
				}
				iTileMap.SetTile(toPoint, checkTileIndex, foundFlop);
				iQueue.push(toPoint);
				return true;
			}
		}
	}
	return false;
}



std::vector<Tile> ReadFile(const std::string& iFileName) {
	std::ifstream ifs(iFileName);
	if (!ifs) {
		std::cerr << "Cannot open file: " << iFileName << std::endl;
		throw std::runtime_error("");
	}
	std::vector<Tile> tiles;
	std::string buf;
	while (std::getline(ifs, buf)) {
		size_t i1 = buf.find_first_of(' ') + 1;
		size_t i2 = buf.find_first_of(':');
		Tile tile;
		tile._tileID = stoi(buf.substr(i1, i2 - i1));
		while (true) {
			std::getline(ifs, buf);
			if (buf.empty()) {
				tiles.push_back(tile);
				break;
			}
			tile._tileData.push_back(buf);
		}
	}
	return tiles;
}




int CalculatePart2(const TileMap& iTileMap, const std::vector<Tile>& iTiles) {
	std::vector<std::string> monster{
		"..................#.",
		"#....##....##....###",
		".#..#..#..#..#..#..."
	};
	int nrMonsterPixels = 0;
	for (const auto& monsterLine : monster) {
		nrMonsterPixels += std::count(monsterLine.begin(), monsterLine.end(), '#');
	}
	int monsterWidth = monster[0].size();
	int monsterHeight = monster.size();

	int tileWidth = iTiles[0]._tileData.size();
	int bigmapWidth = iTileMap._gridWidth * (tileWidth - 2);

	int totalCount = 0;
	for (int y = 0; y < bigmapWidth; ++y) {
		for (int x = 0; x < bigmapWidth; ++x) {
			if (iTileMap.GetBigPixel(x, y, 0) == '#') {
				++totalCount;
			}
		}
	}

	int nrMatches = 0;
	for (int checkFlop = 0; checkFlop <= 7; ++checkFlop) {
		for (int checkY = 0; checkY < (bigmapWidth - monsterHeight); ++checkY) {
			for (int checkX = 0; checkX < (bigmapWidth - monsterWidth); ++checkX) {
				bool isMatch = true;
				for (int y = 0; y < monsterHeight; ++y) {
					for (int x = 0; x < monsterWidth; ++x) {
						if (monster[y][x] == '#' && iTileMap.GetBigPixel(checkX + x, checkY + y, checkFlop) != '#') {
							isMatch = false;
							break;
						}
					}
					if (!isMatch) break;
				}
				if (isMatch) ++nrMatches;
			}
		}
	}

	int result = totalCount - nrMatches * nrMonsterPixels;
	return result;
}



int main()
{
	const auto& tiles = ReadFile("../../../inputs/day20_input.txt");
	int gridWidth = sqrt(tiles.size());
	TileMap tileMap(tiles, gridWidth);

	Point p{ gridWidth, gridWidth };
	tileMap.SetTile(p, 0, 0);

	std::queue<Point> check;
	check.push(p);
	while (!check.empty()) {
		auto checkPoint = check.front();
		check.pop();
		for (int dir = 0; dir < 4; ++dir) {
			FindTile(check, tiles, tileMap, checkPoint, static_cast<EdgeType>(dir));
		}
	}
	tileMap.FinishAndCalculateMap();
	int64_t product = tileMap.CalculateCornerIDProduct();
	std::cout << "Part 1 product: " << product << std::endl;
	assert(product == 29293767579581);

	int part2 = CalculatePart2(tileMap, tiles);
	std::cout << "Part 2: " << part2 << std::endl;
	assert(part2 == 1989);

	return 0;
}
