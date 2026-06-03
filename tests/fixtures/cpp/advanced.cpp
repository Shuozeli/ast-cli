#include <string>
#include <vector>
#include <memory>
#include <functional>

// ── Nested namespaces ───────────────────────────────────
namespace engine {
namespace core {

// ── Template class with multiple params ─────────────────
template <typename T, size_t N>
class FixedBuffer {
public:
    FixedBuffer() : size_(0) {}

    void push(const T& value) {
        if (size_ < N) {
            data_[size_++] = value;
        }
    }

    const T& at(size_t index) const {
        return data_[index];
    }

    size_t size() const { return size_; }

private:
    T data_[N];
    size_t size_;
};

// ── Abstract base class with virtuals ───────────────────
class Component {
public:
    virtual ~Component() = default;

    virtual void update(double dt) = 0;
    virtual std::string name() const = 0;

    int id() const { return id_; }

protected:
    int id_ = 0;
};

// ── Multiple inheritance ────────────────────────────────
class Serializable {
public:
    virtual std::string serialize() const = 0;
};

class Transform : public Component, public Serializable {
public:
    // ── Nested class ────────────────────────────────────
    class Matrix {
    public:
        float data[16];
        Matrix() : data{} {}
        float determinant() const { return 1.0f; }
        friend class Transform;
    };

    Transform() : position_{0, 0, 0} {}

    void update(double dt) override {
        position_[0] += static_cast<float>(dt);
    }

    std::string name() const override {
        return "Transform";
    }

    std::string serialize() const override {
        return "{}";
    }

    void set_position(float x, float y, float z) {
        position_[0] = x;
        position_[1] = y;
        position_[2] = z;
    }

private:
    float position_[3];
};

} // namespace core

// ── Using alias ─────────────────────────────────────────
using ComponentPtr = std::unique_ptr<core::Component>;

// ── Concept ─────────────────────────────────────────────
template<typename T>
concept Integral = std::is_integral_v<T>;

// ── Union ───────────────────────────────────────────────
union VariantData {
    int i;
    float f;
};

// ── Typedef ─────────────────────────────────────────────
typedef std::vector<ComponentPtr> ComponentList;

// ── Free template function ──────────────────────────────
template <typename T>
T clamp(T value, T min_val, T max_val) {
    if (value < min_val) return min_val;
    if (value > max_val) return max_val;
    return value;
}

// ── constexpr function ──────────────────────────────────
constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}

// ── Operator overload ───────────────────────────────────
class Vec2 {
public:
    float x, y;
    Vec2(float x, float y) : x(x), y(y) {}
    Vec2 operator+(const Vec2& other) const {
        return Vec2(x + other.x, y + other.y);
    }
    bool operator==(const Vec2& other) const {
        return x == other.x && y == other.y;
    }
};

} // namespace engine

// ── Preprocessor-wrapped definition ─────────────────────
#ifdef ENABLE_LOGGING
namespace logging {
    void log(const std::string& msg) {
        // stub
    }
}
#endif

// ── Anonymous namespace ─────────────────────────────────
namespace {
    int internal_counter = 0;

    void increment() {
        ++internal_counter;
    }
}
