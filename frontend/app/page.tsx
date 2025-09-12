"use client";

import { useState } from "react";
import { Search, Trophy, Sword, Shield, Users, Zap } from "lucide-react";

interface Player {
  tag: string;
  name: string;
  expLevel: number;
  trophies: number;
  bestTrophies: number;
  wins: number;
  losses: number;
  battleCount: number;
  threeCrownWins: number;
  winStreak: number;
  bestWinStreak: number;
  totalDonations: number;
  clan?: {
    tag: string;
    name: string;
    badgeId: number;
  };
  arena: {
    id: number;
    name: string;
  };
}

export default function Home() {
  const [playerTag, setPlayerTag] = useState("");
  const [player, setPlayer] = useState<Player | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const searchPlayer = async () => {
    if (!playerTag.trim()) return;
    
    setLoading(true);
    setError("");
    setPlayer(null);

    try {
      // Ensure the tag has # prefix and URL encode it
      const tagWithHash = playerTag.startsWith("#") ? playerTag : `#${playerTag}`;
      const encodedTag = encodeURIComponent(tagWithHash);
      const response = await fetch(`http://127.0.0.1:8080/api/player/${encodedTag}`);
      
      if (!response.ok) {
        let errorMessage = "Failed to fetch player data";
        try {
          const errorData = await response.json();
          errorMessage = errorData.message || errorData.error || errorMessage;
        } catch (parseError) {
          errorMessage = `Server error: ${response.status} ${response.statusText}`;
        }
        throw new Error(errorMessage);
      }

      const playerData = await response.json();
      setPlayer(playerData);
    } catch (err) {
      console.error("Fetch error:", err);
      if (err instanceof TypeError && err.message.includes("Failed to fetch")) {
        setError("Cannot connect to backend server. Make sure it's running on localhost:8080.");
      } else {
        setError(err instanceof Error ? err.message : "An error occurred");
      }
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      searchPlayer();
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-900 via-purple-900 to-indigo-900">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-6xl font-bold text-white mb-4">
            Clash Royale Stats
          </h1>
          <p className="text-xl text-blue-200 mb-8">
            Track player statistics and battle performance
          </p>
        </div>

        {/* Search Section */}
        <div className="max-w-2xl mx-auto mb-12">
          <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <div className="flex gap-4">
              <div className="flex-1">
                <input
                  type="text"
                  placeholder="Enter player tag (e.g., #2JG82JV8G or 2JG82JV8G)"
                  value={playerTag}
                  onChange={(e) => setPlayerTag(e.target.value)}
                  onKeyPress={handleKeyPress}
                  className="w-full px-4 py-3 rounded-lg bg-white/20 border border-white/30 text-white placeholder-white/70 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
              <button
                onClick={searchPlayer}
                disabled={loading || !playerTag.trim()}
                className="px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg font-semibold transition-colors flex items-center gap-2"
              >
                <Search className="w-5 h-5" />
                {loading ? "Searching..." : "Search"}
              </button>
            </div>
          </div>
        </div>

        {/* Error Message */}
        {error && (
          <div className="max-w-2xl mx-auto mb-8">
            <div className="bg-red-500/20 border border-red-500/50 rounded-lg p-4 text-red-200">
              <p className="font-semibold">Error:</p>
              <p>{error}</p>
            </div>
          </div>
        )}

        {/* Player Data */}
        {player && (
          <div className="max-w-4xl mx-auto">
            <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
              {/* Player Header */}
              <div className="text-center mb-8">
                <h2 className="text-3xl font-bold text-white mb-2">{player.name}</h2>
                <p className="text-blue-200 text-lg">#{player.tag}</p>
                <div className="flex items-center justify-center gap-4 mt-4">
                  <div className="flex items-center gap-2 text-yellow-400">
                    <Trophy className="w-5 h-5" />
                    <span className="font-semibold">Level {player.expLevel}</span>
                  </div>
                  <div className="flex items-center gap-2 text-blue-300">
                    <Shield className="w-5 h-5" />
                    <span className="font-semibold">{player.arena.name}</span>
                  </div>
                </div>
              </div>

              {/* Stats Grid */}
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {/* Trophies */}
                <div className="bg-white/10 rounded-xl p-6 text-center">
                  <Trophy className="w-8 h-8 text-yellow-400 mx-auto mb-3" />
                  <h3 className="text-white font-semibold mb-2">Trophies</h3>
                  <p className="text-2xl font-bold text-yellow-400">{player.trophies.toLocaleString()}</p>
                  <p className="text-sm text-blue-200">Best: {player.bestTrophies.toLocaleString()}</p>
                </div>

                {/* Battle Stats */}
                <div className="bg-white/10 rounded-xl p-6 text-center">
                  <Sword className="w-8 h-8 text-red-400 mx-auto mb-3" />
                  <h3 className="text-white font-semibold mb-2">Battles</h3>
                  <p className="text-2xl font-bold text-red-400">{player.battleCount.toLocaleString()}</p>
                  <p className="text-sm text-blue-200">Wins: {player.wins.toLocaleString()}</p>
                </div>

                {/* Win Rate */}
                <div className="bg-white/10 rounded-xl p-6 text-center">
                  <Zap className="w-8 h-8 text-green-400 mx-auto mb-3" />
                  <h3 className="text-white font-semibold mb-2">Win Rate</h3>
                  <p className="text-2xl font-bold text-green-400">
                    {player.battleCount > 0 ? Math.round((player.wins / player.battleCount) * 100) : 0}%
                  </p>
                  <p className="text-sm text-blue-200">Losses: {player.losses.toLocaleString()}</p>
                </div>

                {/* Streaks */}
                <div className="bg-white/10 rounded-xl p-6 text-center">
                  <Shield className="w-8 h-8 text-purple-400 mx-auto mb-3" />
                  <h3 className="text-white font-semibold mb-2">Streaks</h3>
                  <p className="text-2xl font-bold text-purple-400">{player.winStreak}</p>
                  <p className="text-sm text-blue-200">Best: {player.bestWinStreak}</p>
                </div>
              </div>

              {/* Additional Stats */}
              <div className="mt-8 grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="bg-white/5 rounded-lg p-4">
                  <h4 className="text-white font-semibold mb-2">Three Crown Wins</h4>
                  <p className="text-xl text-yellow-400">{player.threeCrownWins.toLocaleString()}</p>
                </div>
                <div className="bg-white/5 rounded-lg p-4">
                  <h4 className="text-white font-semibold mb-2">Total Donations</h4>
                  <p className="text-xl text-blue-400">{player.totalDonations.toLocaleString()}</p>
                </div>
                <div className="bg-white/5 rounded-lg p-4">
                  <h4 className="text-white font-semibold mb-2">Clan</h4>
                  <p className="text-xl text-green-400">
                    {player.clan ? player.clan.name : "No Clan"}
                  </p>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Instructions */}
        {!player && !loading && (
          <div className="max-w-2xl mx-auto text-center">
            <div className="bg-white/5 rounded-xl p-8 border border-white/10">
              <h3 className="text-xl font-semibold text-white mb-4">How to use:</h3>
              <ol className="text-blue-200 space-y-2 text-left">
                <li>1. Enter a Clash Royale player tag (with or without # symbol)</li>
                <li>2. Click "Search" or press Enter</li>
                <li>3. View detailed player statistics</li>
              </ol>
              <p className="text-sm text-blue-300 mt-4">
                Note: Make sure your Rust backend is running on localhost:8080
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
