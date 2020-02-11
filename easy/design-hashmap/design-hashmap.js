/**
 * Initialize your data structure here.
 */
var MyHashMap = function () {
  this.size = 10000
  this.table = []
};

MyHashMap.prototype.hash = function (key) {
  return key;
}

MyHashMap.prototype.getIndex = function (key) {
  return key % this.size
};

/**
 * value will always be non-negative. 
 * @param {number} key 
 * @param {number} value
 * @return {void}
 */
MyHashMap.prototype.put = function (key, value) {
  const hash = this.hash(key);
  this.table[hash] = value;
  return;
};

/**
 * Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key 
 * @param {number} key
 * @return {number}
 */
MyHashMap.prototype.get = function (key) {
  const hash = this.hash(key);
  if (this.table[hash] >= 0) {
    return this.table[hash];
  }
  return -1;
};

/**
 * Removes the mapping of the specified value key if this map contains a mapping for the key 
 * @param {number} key
 * @return {void}
 */
MyHashMap.prototype.remove = function (key) {
  const hash = this.hash(key);
  delete this.table[hash];
  return;
};

/**
 * Your MyHashMap object will be instantiated and called as such:
 * var obj = new MyHashMap()
 * obj.put(key,value)
 * var param_2 = obj.get(key)
 * obj.remove(key)
 */


