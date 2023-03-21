#pragma once

#include "extra.h"

struct Arenar {
    Arena* a;

    Arenar(GameMode game_mode, float tick_rate = 120) {
        a = Arena::Create(game_mode, tick_rate);
    }

    ~Arenar() {
        delete a;
    }

    // No copy constructor
    Arenar(const Arenar & other) = delete;
    Arenar & operator =(const Arenar & other) = delete;

    // Move constructor
    Arenar(Arenar&& other) = default;
    Arenar& operator =(Arenar && other) = default;

	// extra car stuff
	uint32_t num_cars() const {
		return a->_cars.size();
	}

	uint32_t get_car_id(uint32_t index) const {
		return a->_cars[index]->id;
	}

	CarState GetCarFromIndex(uint32_t index);
	CarState GetCar(uint32_t car_id);
	/// @brief Sets the state of a car in the arena
	/// @param arena
	/// @param state
	/// @param carID
	/// @return True if the car was found and the state was set, false otherwise
	bool SetCar(uint32_t car_id, const CarState& state);

	uint32_t AddCar(Team team, const CarConfig& config) {
		return a->AddCar(team, config)->id;
	}

	bool RemoveCar(uint32_t car_id);
	/// @brief Sets the controls of a car for the next tick
	/// @param arena
	/// @param state
	/// @param carID
	/// @return True if the car was found and the state was set, false otherwise
	bool SetCarControls(uint32_t car_id, const CarControls& controls);
	bool DemolishCar(uint32_t car_id);
	bool RespawnCar(uint32_t car_id, int32_t seed);

	// extra ball stuff

	BallState GetBall() const;
	void SetBall(const BallState& state);

	// boost pad stuff

	uint32_t num_boost_pads() const {
		return a->_boostPads.size();
	}

	bool get_pad_is_big(uint32_t index) const;
	Vec GetPadPos(uint32_t index) const;
	void SetPadState(const EBoostPadState& state);
	EBoostPadState GetPadState(uint32_t index) const;

	// extra misc stuff

	uint64_t get_tick_count() const {
		return a->tickCount;
	}

	float get_tick_rate() const {
		return 1 / a->tickTime;
	}

	void ResetToRandomKickoff(int32 seed = -1) {
		a->ResetToRandomKickoff(seed);
	}

	void step(int32 ticks = 1) {
		a->Step(ticks);
	}
};