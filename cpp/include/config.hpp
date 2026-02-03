#pragma once

#include "models.hpp"
#include <map>

namespace OffsetsFinder {

using GameConfig = std::vector<std::pair<OffsetCategory, std::vector<Target>>>;

GameConfig GetGameConfig(GameVariant variant);

GameConfig GetFreeFire Offsets();

GameConfig GetFreeFireMaxOffsets();

GameConfig GetFreeFireTelaOffsets();

std::optional<GameVariant> DetectGameVariant(const std::string& content);

}
