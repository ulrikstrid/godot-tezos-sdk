extends GutTest

var packed_string_1 = "050100000037556e6974792054657a6f73204578616d706c652050726f6a65637420636f696e73207573656420617320736f66742063757272656e6379"
var packed_string_2 = "050100000042697066733a2f2f62616679626569616e32336f646873686f3667756661637263706372363566743662707161767a6b3336707432326c68636a6f787934356d717061"
var unpacked_string_1 = "{\"string\":\"Unity Tezos Example Project coins used as soft currency\"}"
var unpacked_string_2 = "{\"string\":\"Example Coin\"}"

func test_unpack():
	var ds_1 = TezosMichelson.unpack(packed_string_1)
	print(ds_1)
	assert_eq(ds_1, unpacked_string_1)	

	var ds_2 = TezosMichelson.unpack(packed_string_2)
	print(ds_2)
	assert_eq(ds_2, unpacked_string_2)

func test_encode():
	var es_1 = TezosMichelson.pack(unpacked_string_1)
	print(es_1)
	assert_eq(es_1, packed_string_1)

	var es_2 = TezosMichelson.pack(unpacked_string_2)
	print(es_2)
	assert_eq(es_2, packed_string_2)

func test_unpack_to_json():
	var ds = TezosMichelson.unpack(packed_string_2)

	var json = JSON.parse_string(ds)
	# TODO: Can we make this nicer?
	assert_eq(json["string"], "Example Coin")

func test_unsigned_operation():
	var uo = TezosOperation.make_unsigned()
	assert_eq(uo, "test")

func test_parameters():
	pass